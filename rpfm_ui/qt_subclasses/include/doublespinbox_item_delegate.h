#ifndef DOUBLESPINBOX_ITEM_DELEGATE_H
#define DOUBLESPINBOX_ITEM_DELEGATE_H

#include "qt_subclasses_global.h"
#include <QStyledItemDelegate>
#include <QAbstractItemDelegate>
#include <QDoubleSpinBox>

extern "C" void new_double_spinbox_item_delegate(QObject *parent = nullptr, const int column = 0);

class QDoubleSpinBoxItemDelegate : public QStyledItemDelegate
{
    Q_OBJECT

public:

    explicit QDoubleSpinBoxItemDelegate(QObject *parent = nullptr);

    QWidget* createEditor(QWidget *parent, const QStyleOptionViewItem &, const QModelIndex &) const;
    void setEditorData(QWidget *editor, const QModelIndex &index) const;
    void setModelData(QWidget *editor, QAbstractItemModel *model, const QModelIndex &index) const;
    void updateEditorGeometry(QWidget *editor, const QStyleOptionViewItem &option, const QModelIndex &) const;

signals:

private:
};

#endif // DOUBLESPINBOX_ITEM_DELEGATE_H
